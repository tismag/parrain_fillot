# Programme d'association parrains/fillots

### Conditions de bon fonctionnement

-   Plus les questions ont d'options de réponse, moins l'algorithme est efficace
        => Prioriser le nombre de questions au nombre d'options de réponses

-   Prévu pour plus de fillots que de parrains

-   Même questionnaire pour les parrains et les fillots

-   Le CSV doit contenir une 1ère ligne et une 1ère colonne inutiles (le programme se charge de les enlever)
        => Il est prévu l'utilisation de Google Form pour le questionnaire.
        En export csv depuis Google Form, la 1ère ligne réitère les questions 
        et la première colonne contient l'horodateur, on ne veut pas ces données

-   Pas de virgules ni de saut de ligne dans les options de réponse
    
### Usage

Avant de lancer l'exécution vous devrez placer deux fichiers dans le répertoire parrain-fillot cloné,
    - parrains.csv
    - fillots.csv
    
contenant chacun les réponses aux questionnaires des parrains et fillots.

##### Exemple
```
Horodateur,Prénom,Nom,Filière,Bringue ?,Êtes-vous alcoolique et pourquoi oui ?
30/06/2022 16:15:58,Jean,JeanJean,JEAN,Non,Bonne question
30/06/2022 17:06:39,Corinne,Frein,PRI,Non,Oui et ?
...
```

Pour lancer l'exécution (sous Linux uniquement pour l'instant), exécuter la commande
`cargo run <Nombre de questions type Informations personnelles dans le questionnaire>`

Pour notre exemple le nombre à donner est 3 (Prénom, Nom, Filière)

### Sortie du programme

L'exécution du programme génère un fichier parrains_fillots.txt. Dans certains cas des parrains peuvent se retrouver sans fillots selon les réponses. Ce n'est pas un souci comme il est prévu de repasser sur les données en sortie pour adapter les résultats selon les éventuelles requêtes des parrains.
