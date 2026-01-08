# Pet Foster Connect

<img src="./assets/icons/logo.svg" alt="Logo de PetFosterConnect" width="300"/>

## Disclaimer

Ce dépôt contient une conversion de l'API utilisée par PetFosterConnect en langage Rust, s'appuyant sur le framework Actix Web tout en conservant les mêmes fonctionnalités.

## Présentation

Pet Foster Connect permet de mettre en relation des familles d’accueil pour les animaux avec des associations de protection animale.

PFC permet aux gens de jouer un rôle fondamental en accueillant des animaux en attendant leur adoption définitive afin de leur offrir une meilleure vie.

PFC a pour vocation de répondre à plusieurs besoins :

- Les animaux aimeraient bien un toit, et les gens aiment les animaux (en général)
- Permettre aux associations / refuges de communiquer sur les animaux nécessitant une place au chaud
- Permettre aux familles d'accueil de se faire connaître et de se mettre en relation avec les refuges / associations

## Technologies utilisées

Pour réaliser cette application, nous nous sommes servis de :

| **Nom** | **Utilité** |
| -------------- | ---------------- |
| Rust | Langage |
| Actix Wev | Framework |
| PostgreSQL | Base de données |
| SeaORM | ORM |
| Chrono | Dates |
| Dotenv | Environnement |
| env_logger & log | Historique |
| Regex | Expressions régulières |
| Once Cell | Initialisation de regex |
| Bcrypt | Algorithme de hachage |
| JsonWebToken | JWT |
| Serde | Sérialisation |
| Serde JSON | Création de JSON |
| UUID | Génération d'Id |
| Validator | Validation de champs |
