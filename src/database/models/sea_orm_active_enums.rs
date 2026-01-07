use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, serde::Serialize, serde::Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sexe")]
pub enum Sexe {
    #[sea_orm(string_value = "Mâle")]
    Mâle,
    #[sea_orm(string_value = "Femelle")]
    Femelle,
    #[sea_orm(string_value = "Inconnu")]
    Inconnu,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, serde::Serialize, serde::Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "statut")]
pub enum Statut {
    #[sea_orm(string_value = "En refuge")]
    EnRefuge,
    #[sea_orm(string_value = "Accueilli")]
    Accueilli,
    #[sea_orm(string_value = "Adopté")]
    Adopté,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, serde::Serialize, serde::Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "statut_demande")]
pub enum StatutDemande {
    #[sea_orm(string_value = "En attente")]
    EnAttente,
    #[sea_orm(string_value = "Validée")]
    Validée,
    #[sea_orm(string_value = "Refusée")]
    Refusée,
}
