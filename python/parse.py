def parse_csv(line):
    """
    Retourne un tuple contenant un arc sous forme de tuple.
    >>> parse_csv("1,2")
    (1, 2)
    """
    v = line.split(",")
    return (int(v[0]), int(v[1]))


def parse_txt(line):
    """
    Retourne un tuple contenant un arc sous forme de tuple.
    >>> parse_txt("1\t2")
    (1, 2)
    """
    v = line.split()
    return (int(v[0]), int(v[1]))


def save_csv(f, arc):
    """
    Save in f the arc in CSV format
    """
    f.write("{},{}\n".format(arc[0], arc[1]))


def save_txt(f, arc):
    """
    Save in f the arc in txt (with tab) format
    """
    f.write("{}	{}\n".format(arc[0], arc[1]))
