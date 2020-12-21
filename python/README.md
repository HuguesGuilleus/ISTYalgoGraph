# ISTYalgoGraph (Python)

Nous utilisons la version 3 de Python.

## Utilisations

-   Test unitaire: `python3 -m doctest graph.py parse.py`
-   Génération de la documentation: `pydoc3 -w graph parse`
-   Lancement comme programme: `python3 graph.py [ARG]`

```txt
Charge ou genère le graphe avec:
    gg|gen_gilbert         size
    gb|gen_barabasi_albert size
    l|load                 file.csv|file.txt

Puis indiquez un fichier si vous voulez exporter le graphe,
sinon ses statistiques seront affichées.

Exemple:
$ python3 graph.py load ../db/Wikipedia2.csv
```

## Fichiers

-   `graph.py`: Contient la classe `Graph`, peut-être utilisé comme librairie ou comme script.
-   `parse.py`: Sert à sérialiser et désérialiser les graphe stockés en `.txt` ou `.csv`.
