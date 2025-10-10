use bincode::{Decode, Encode};

use super::ErrorCode;

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub enum ResponseStatus<T> {
    /// Opération réussie avec les données
    Success { data: T },

    /// Opération échouée avec un code d'erreur et un message
    Error { code: ErrorCode, message: String },

    /// Opération partiellement réussie avec données, warning et progression
    Partial {
        data: T,
        warning: String,
        progress: f32, // 0.0 à 1.0 (0% à 100%)
    },
}

impl<T> ResponseStatus<T> {
    /// Crée une réponse Success
    pub fn success(data: T) -> Self {
        ResponseStatus::Success { data }
    }

    /// Crée une réponse Error
    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        ResponseStatus::Error {
            code,
            message: message.into(),
        }
    }

    /// Crée une réponse Partial
    pub fn partial(data: T, warning: impl Into<String>, progress: f32) -> Self {
        ResponseStatus::Partial {
            data,
            warning: warning.into(),
            progress: progress.clamp(0.0, 1.0),
        }
    }

    /// Vérifie si la réponse est un succès
    pub fn is_success(&self) -> bool {
        matches!(self, ResponseStatus::Success { .. })
    }

    /// Vérifie si la réponse est une erreur
    pub fn is_error(&self) -> bool {
        matches!(self, ResponseStatus::Error { .. })
    }

    /// Vérifie si la réponse est partielle
    pub fn is_partial(&self) -> bool {
        matches!(self, ResponseStatus::Partial { .. })
    }

    /// Récupère les données si Success ou Partial, None sinon
    pub fn data(self) -> Option<T> {
        match self {
            ResponseStatus::Success { data } => Some(data),
            ResponseStatus::Partial { data, .. } => Some(data),
            ResponseStatus::Error { .. } => None,
        }
    }

    /// Récupère une référence aux données si Success ou Partial
    pub fn data_ref(&self) -> Option<&T> {
        match self {
            ResponseStatus::Success { data } => Some(data),
            ResponseStatus::Partial { data, .. } => Some(data),
            ResponseStatus::Error { .. } => None,
        }
    }
}
