# L'objectif. 

Faire une estimation de la consomation de ressources de la CI/CD d'un projet

## Utilisation

- git clone
- cargo run 

## Variable d'environnement: 
- OWNER: Nom d'utilisateur du répo cible. 
- REPO: repo a analyser
- TOKEN: token github pour accès aux répo privé

## Estimation du cout d'un projet


### Estimation Espace disque

- Taille du projet
- Tailles des artifacts et du cache

### Estimation réseau

- Nombre de Workflow qui passe * Taille du projet

