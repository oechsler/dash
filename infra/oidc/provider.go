package oidc

import (
	"context"
	"github.com/oechsler-it/dash/domain/model"
	"github.com/oechsler-it/dash/config"
	"fmt"
	"net/url"
	"strings"

	"github.com/coreos/go-oidc/v3/oidc"
	"github.com/samber/lo"
	"golang.org/x/oauth2"
)

// Provider wraps the OIDC provider and OAuth2 configuration.
// It handles discovery, token exchange, and claim extraction.
type Provider struct {
	oidcProvider  *oidc.Provider
	verifier      *oidc.IDTokenVerifier
	oauth2Config  oauth2.Config
	adminGroup    string
	profileUrl    string
	endSessionURL string
}

type providerClaims struct {
	EndSessionEndpoint string `json:"end_session_endpoint"`
}

// NewProvider initialises the OIDC provider via discovery (/.well-known/openid-configuration).
func NewProvider(ctx context.Context, cfg *config.OIDCConfig) (*Provider, error) {
	oidcProvider, err := oidc.NewProvider(ctx, cfg.Issuer)
	if err != nil {
		return nil, fmt.Errorf("OIDC discovery failed for %s: %w", cfg.Issuer, err)
	}

	// Auto-discover end_session_endpoint from provider metadata.
	var claims providerClaims
	_ = oidcProvider.Claims(&claims) // non-fatal if field absent

	endSessionURL := claims.EndSessionEndpoint
	if cfg.EndSessionURL != "" {
		endSessionURL = cfg.EndSessionURL
	}

	scopes := strings.Fields(cfg.Scopes)

	verifier := oidcProvider.Verifier(&oidc.Config{ClientID: cfg.ClientID})

	oauth2Config := oauth2.Config{
		ClientID:     cfg.ClientID,
		ClientSecret: cfg.ClientSecret,
		RedirectURL:  cfg.RedirectURL,
		Endpoint:     oidcProvider.Endpoint(),
		Scopes:       scopes,
	}

	return &Provider{
		oidcProvider:  oidcProvider,
		verifier:      verifier,
		oauth2Config:  oauth2Config,
		adminGroup:    cfg.AdminGroup,
		profileUrl:    cfg.ProfileURL,
		endSessionURL: endSessionURL,
	}, nil
}

// BeginAuth returns the authorization URL with PKCE S256 challenge and state.
func (p *Provider) BeginAuth(state, codeVerifier string) string {
	return p.oauth2Config.AuthCodeURL(state, oauth2.S256ChallengeOption(codeVerifier))
}

// Exchange performs the authorization code exchange, verifies the ID token against JWKS,
// and returns the verified IDToken and the raw ID token string (needed for logout).
func (p *Provider) Exchange(ctx context.Context, code, codeVerifier string) (*oidc.IDToken, string, error) {
	token, err := p.oauth2Config.Exchange(ctx, code, oauth2.VerifierOption(codeVerifier))
	if err != nil {
		return nil, "", fmt.Errorf("token exchange: %w", err)
	}

	rawIDToken, ok := token.Extra("id_token").(string)
	if !ok {
		return nil, "", fmt.Errorf("no id_token in token response")
	}

	idToken, err := p.verifier.Verify(ctx, rawIDToken)
	if err != nil {
		return nil, "", fmt.Errorf("id_token verification: %w", err)
	}

	return idToken, rawIDToken, nil
}

// EndSessionURL builds the OIDC end_session_endpoint URL for logout.
// Returns empty string if no end_session_endpoint is known.
func (p *Provider) EndSessionURL(idTokenHint, postLogoutRedirectURI string) string {
	if p.endSessionURL == "" {
		return ""
	}
	u, err := url.Parse(p.endSessionURL)
	if err != nil {
		return ""
	}
	q := u.Query()
	if idTokenHint != "" {
		q.Set("id_token_hint", idTokenHint)
	}
	if postLogoutRedirectURI != "" {
		q.Set("post_logout_redirect_uri", postLogoutRedirectURI)
	}
	u.RawQuery = q.Encode()
	return u.String()
}

// ClaimsToIdentity extracts standard OIDC claims from the verified ID token
// and maps them to the domain Identity value object.
func (p *Provider) ClaimsToIdentity(idToken *oidc.IDToken) (model.Identity, error) {
	var claims struct {
		Sub               string   `json:"sub"`
		GivenName         string   `json:"given_name"`
		FamilyName        string   `json:"family_name"`
		PreferredUsername string   `json:"preferred_username"`
		Email             string   `json:"email"`
		Picture           string   `json:"picture"`
		Groups            []string `json:"groups"`
	}
	if err := idToken.Claims(&claims); err != nil {
		return model.Identity{}, fmt.Errorf("extracting claims: %w", err)
	}

	// UserID: prefer preferred_username → sub → email
	userID := claims.PreferredUsername
	if userID == "" {
		userID = claims.Sub
	}
	if userID == "" {
		userID = claims.Email
	}

	// FirstName fallback
	firstName := claims.GivenName
	if firstName == "" {
		firstName = claims.PreferredUsername
	}

	// DisplayName
	displayName := claims.PreferredUsername
	if claims.GivenName != "" || claims.FamilyName != "" {
		name := claims.GivenName
		if name == "" {
			name = claims.PreferredUsername
		}
		if claims.FamilyName != "" {
			displayName = name + " " + claims.FamilyName
		} else {
			displayName = name
		}
	}

	isAdmin := p.adminGroup == "*" || lo.Contains(claims.Groups, p.adminGroup)

	var picture *string
	if claims.Picture != "" {
		picture = &claims.Picture
	}

	var profileUrl *string
	if p.profileUrl != "" {
		pu := p.profileUrl
		profileUrl = &pu
	}

	return model.Identity{
		UserID:      userID,
		FirstName:   firstName,
		LastName:    claims.FamilyName,
		DisplayName: displayName,
		Username:    claims.PreferredUsername,
		Email:       claims.Email,
		Picture:     picture,
		Groups:      claims.Groups,
		IsAdmin:     isAdmin,
		ProfileUrl:  profileUrl,
	}, nil
}
