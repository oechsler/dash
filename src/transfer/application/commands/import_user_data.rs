use crate::{
    shared::application::ApplicationError,
    transfer::application::{
        ports::{UserDataReader, UserDataWriter},
        services::{ImportDataService, ImportFormatService},
    },
    user::domain::UserId,
};

pub struct ImportUserDataInput {
    pub owner: UserId,
    pub is_admin: bool,
    pub payload: Vec<u8>,
}

pub struct ImportUserData<R, W> {
    reader: R,
    writer: W,
    importer: ImportFormatService,
    apply_import: ImportDataService,
}

impl<R: UserDataReader, W: UserDataWriter> ImportUserData<R, W> {
    pub fn new(
        reader: R,
        writer: W,
        importer: ImportFormatService,
        apply_import: ImportDataService,
    ) -> Self {
        Self {
            reader,
            writer,
            importer,
            apply_import,
        }
    }

    pub async fn execute(&self, input: ImportUserDataInput) -> Result<(), ApplicationError> {
        let imported = self.importer.import(&input.payload, &input.owner)?;
        let mut snapshot = self.reader.load(&input.owner).await?;
        self.apply_import
            .apply(&mut snapshot, &input.owner, imported, input.is_admin)?;
        self.writer.save(&input.owner, &snapshot).await
    }
}
