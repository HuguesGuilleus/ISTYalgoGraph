def parse_csv(line):
    """
    Retourne un tuple contenant un arc sous forme de tuple.
    >>> parse_csv("1,2")
    (1, 2)
    """
    v = line.split(",")
    return (int(v[0]), int(v[1]))


def load_csv(f, save):
    """
    Prend un fichier `f` et une function `save` pour enregistrer dans le graphe
    les arcs.
    """
    for line in f.readlines(0)[1:]:
        save(parse_csv(line[:-1]))


def parse_txt(line):
    """
    Retourne un tuple contenant un arc sous forme de tuple.
    >>> parse_txt("1\t2")
    (1, 2)
    """
    v = line.split()
    return (int(v[0]), int(v[1]))


def load_txt(f, save):
    """
    Prend un fichier `f` et une function `save` pour enregistrer dans le graphe
    les arcs.
    """
    for l in f.readlines(0):
        try:
            l = l[: l.index("#")]
        except:
            pass
        l = l.strip()

        if len(l):
            save(parse_txt(l))


def save_csv(f, arc):
    """
    Save in f the arc in CSV format
    """
    f.write("{},{}\n".format(arc[0], arc[1]))


def save_txt(f, arc):
    """
    Save in f the arc in txt (with tab) format
    """
    f.write("{}\t{}\n".format(arc[0], arc[1]))
