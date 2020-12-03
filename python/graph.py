import matplotlib.pyplot as plt
import parse
import secrets


class Graph:
    """
    # Source: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples
    >>> g = Graph(8)
    >>> vertices = [(0, 1), (0, 4), (1, 6), (3, 6), (3, 2), (5, 0), (5, 1), (5, 2), (7, 6), (7, 6)]
    >>> for v in vertices: g.add_edge(v)
    >>> g
    * 1 . . 1 1 . .
    1 * . . . 1 1 .
    . . * 1 . 1 . .
    . . 1 * . . 1 .
    1 . . . * . . .
    1 1 1 . . * . .
    . 1 . 1 . . * 2
    . . . . . . 2 *
    >>> g.bfs(5)
    [1, 1, 1, 2, 2, 0, 2, 3]
    >>> g.calc_distance(False)
    4
    """

    def __init__(self, size):
        self.n = size
        self.adjacency_list = [[] for i in range(size)]
        self.nb_edges_tot = 0
        self.degree = [0] * size
        self.max_degree = 0
        self.average_degree = 0

    def __repr__(self):
        s = ""
        for line, child in enumerate(self.adjacency_list):
            nodes = child.copy()
            nodes.sort()
            i = 0
            for j in range(0, self.n):
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

    def grow_with_edge(self, vertices):
        """Ajoute une arrete et grossit le graphe si besoin"""
        m = max(vertices[0], vertices[1]) + 1
        if self.n < m:
            for i in range(self.n, m):
                self.adjacency_list.append([])
                self.degree.append(0)
            self.n = m
        self.adjacency_list[vertices[0]].append(vertices[1])
        self.adjacency_list[vertices[1]].append(vertices[0])
        self.degree[vertices[0]] += 1
        self.degree[vertices[1]] += 1
        self.nb_edges_tot += 1

    def add_edge(self, vertices):
        """Ajoute une arrete"""
        self.adjacency_list[vertices[0]].append(vertices[1])
        self.adjacency_list[vertices[1]].append(vertices[0])
        self.degree[vertices[0]] += 1
        self.degree[vertices[1]] += 1
        self.nb_edges_tot += 1

    def gen_gilbert(self):
        n = self.n
        for i in range(0, n):
            for j in range(i + 1, n):
                r = secrets.randbits(1)
                if r:
                    self.add_edge([i, j])

    def gen_barabasi_albert(self, m=2):
        # initialisation de la clique
        self.add_edge([0, 1])
        self.add_edge([0, 2])
        self.add_edge([1, 2])

        for i in range(3, self.n):
            cpt = 0
            j = 0
            while cpt < m and j < i:
                r = secrets.randbelow(self.nb_edges_tot * 2)
                if r <= self.degree[j]:
                    self.add_edge([i, j])
                    cpt += 1
                j += 1

    def load(f, size=None):
        """
        Charge les aretes a partir fichier le format est determine par les
        extentions qui penvent etre ".txt" ou bien ".csv".
        >>> lines = ["id_1,id_2\\n", "0,1\\n", "0,2\\n", "1,2\\n"]
        >>> with open("x.csv", "w") as f: f.writelines(lines) ;
        >>> g = Graph.load("x.csv")
        >>> g
        * 1 1
        1 * 1
        1 1 *
        >>> import os; os.remove("x.csv")
        """
        if size == None:
            g = Graph(0)
            save = g.grow_with_edge
        else:
            g = Graph(size)
            save = g.add_edge

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
        >>> g = Graph(3)
        >>> for v in [(0, 1), (1, 2), (0, 2)]: g.add_edge(v)
        >>> g.save("x.csv")
        >>> with open("x.csv", "r") as f: print(f.read())
        id_1,id_2
        0,1
        0,2
        1,2
        >>> import os; os.remove("x.csv")
        """
        if out.endswith(".csv"):
            first = "id_1,id_2"
            saver = parse.save_csv
        elif out.endswith(".txt"):
            first = "# from\tto"
            saver = parse.save_txt
        else:
            raise Exception("Unknown extention to find a save format")

        with open(out, "w") as f:
            f.write(first)
            for p, childs in enumerate(self.adjacency_list):
                for c in childs:
                    if p < c:
                        saver(f, (p, c))

    def curve_distrib_degree(self):
        """Affiche une fenetre contenant le graphique la frequence
        d'appartition des degrees"""
        x = []
        y = [0] * (self.max_degree + 1)
        for i in range(0, self.max_degree + 1):
            x.append(i)
        for i in range(0, self.n):
            y[self.degree[i]] += 1
        plt.bar(x, y)
        plt.show()

    def stat(self):
        """Calcule et affiche les statistiques du graphes."""
        print("1) le nombre de sommets est : " + str(self.n))
        print("2) le nombre d'arêtes est : " + str(self.nb_edges_tot))
        self.max_degree = max(self.degree)
        print("3) le degrée maximal est : " + str(self.max_degree))
        self.average_degree = sum(self.degree) / self.n
        print("4) le degrée moyen est : " + str(self.average_degree))
        print("6) le diamètre du graph est : " + str(self.calc_distance()))
        print("5) la courbe de distribution des degrées s'ouvre dans une fenêtre")
        self.curve_distrib_degree()

    def calc_distance(self, printer=True):
        """Calcule le diametre en cherchant le plus long court chemin en
        partant de chaque sommet. Complexite: O(S*(A+S)). L'argument `printer`
        permet de desactiver l'affichage du sommet en cours de traitement."""
        max = None
        for origin in range(self.n):
            if printer:
                print(f"origin: {origin:,}", end="\x1b[1G")
            for long in self.bfs(origin):
                if max == None:
                    max = long
                elif long and long > max:
                    max = long

        if printer:
            print("\x1b[K", end="")

        return max

    def bfs(self, origin):
        """Applique l'algorithme de parcours en largeur (*Breadth-first search*
        en anglais) sur le sommet `origin`. Complexite: O(A+S)."""
        dist = [None] * self.n
        dist[origin] = 0
        node_todo = [origin]

        while len(node_todo):
            parent = node_todo.pop(0)
            minimum = dist[parent] + 1
            for child in self.adjacency_list[parent]:
                if dist[child] == None:
                    dist[child] = minimum
                    node_todo.append(child)

        return dist
