import secrets
import parse


class Graph:
    """Un graphe, il contient une liste où chaque sommet a ses enfants."""

    def __init__(self, size):
        self.adjacency_list = []
        for i in range(size or 0):
            self.adjacency_list.append([])

    def load(f, size):
        g = Graph(size)
        if size == None:
            save = g.push
        else:
            save = g.add

        if f.endswith(".csv"):
            loader = parse.load_csv
        elif f.endswith(".txt"):
            loader = parse.load_txt
        else:
            raise Exception("Unknown extention to find a loader")

        with open(f, "r") as f:
            loader(f, save)

        return g

    def save(self, out):
        """
        Enregistre le graphe dans le fichier out; le format est déterminé par
        les extentions qui penvent être ".txt" ou bien ".csv".
        """
        if out.endswith(".csv"):
            fisrt = "from,to\n"
            saver = parse.save_csv
        elif out.endswith(".txt"):
            fisrt = "# from\tto\n"
            saver = parse.save_txt
        else:
            raise Exception("Unknown extention to find a saver")

        with open(out, "w") as f:
            f.write(fisrt)
            for p, childs in enumerate(self.adjacency_list):
                for c in childs:
                    saver(f, (p, c))

    def __repr__(self):
        """
        >>> g = Graph(8);
        >>> g.add((0, 1));
        >>> g.add((0, 4));
        >>> g.add((1, 6));
        >>> g.add((3, 6));
        >>> g.add((3, 2));
        >>> g.add((5, 0));
        >>> g.add((5, 1));
        >>> g.add((5, 2));
        >>> g.add((7, 6));
        >>> g.add((7, 6));
        >>> g
        * 1 . . 1 . . .
        . * . . . . 1 .
        . . * . . . . .
        . . 1 * . . 1 .
        . . . . * . . .
        1 1 1 . . * . .
        . . . . . . * .
        . . . . . . 2 *
        """
        s = ""
        for line, child in enumerate(self.adjacency_list):
            nodes = child.copy()
            nodes.sort()
            i = 0
            for j in range(0, self.len()):
                nb = 0
                while i < len(nodes) and nodes[i] == j:
                    nb += 1
                    i += 1
                if nb == 0:
                    if j == line:
                        s += "* "
                    else:
                        s += ". "
                else:
                    s += str(nb) + " "
            s = s[:-1] + "\n"
        return s[:-1]

    def add(self, arc):
        """Ajoute l'arc au graphe si le sommet de départ et d'arrivé sont dans le graphe."""
        begin = arc[0]
        end = arc[1]
        l = self.len()
        if begin >= l or end >= l:
            return
        self.adjacency_list[begin].append(end)

    def push(self, arc):
        begin = arc[0]
        end = arc[1]
        l = self.len()
        m = max(begin, end) + 1
        if m > l:
            for i in range(l, m):
                self.adjacency_list.append([])
        self.adjacency_list[begin].append(end)

    def gen_gilbert(size):
        """Génération de graphe avec le modèle d'Edgar Gilbert."""
        g = Graph(size)
        for i in range(size):
            for j in range(size):
                if secrets.randbits(1):
                    g.add((i, j))
        return g

    def len(self):
        """Retourne le nombre de sommet du graphe. Complexité O(1)
        >>> g = Graph(0); g.push((2,1)); g.add((0,2)); g.len()
        3
        """
        return len(self.adjacency_list)

    def edges(self):
        """Retourne le nombre total d'arrêtes du graphe. Complexité O(S)
        >>> g = Graph(3); g.push((2,1)); g.add((0,2)); g.edges()
        2
        """
        sum = 0
        for childs in self.adjacency_list:
            sum += len(childs)
        return sum
