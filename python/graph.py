import matplotlib.pyplot as plt
import parse
import secrets
import sys


class Graph:
    """
    # Source: https://fr.wikipedia.org/wiki/Matrice_d%27adjacence#Exemples
    >>> g = Graph(8)
    >>> for v in [(0, 1), (0, 4), (1, 6), (3, 6), (3, 2), (5, 0), (5, 1), (5, 2), (7, 6), (7, 6)]: g.add_edge(v)
    >>> g
    * 1 . . 1 1 . .
    1 * . . . 1 1 .
    . . * 1 . 1 . .
    . . 1 * . . 1 .
    1 . . . * . . .
    1 1 1 . . * . .
    . 1 . 1 . . * 2
    . . . . . . 2 *
    >>> g.bfs(5, [True] * g.n)
    [1, 1, 1, 2, 2, 0, 2, 3]
    >>> list(g.children(0, [False, False] + [True] * 6))
    [4, 5]
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

    def load(self, f):
        """
        Charge les aretes a partir fichier le format est determine par les
        extentions qui penvent etre ".txt" ou bien ".csv".
        >>> lines = ["id_1,id_2\\n", "0,1\\n", "0,2\\n", "1,2\\n"]
        >>> with open("x.csv", "w") as f: f.writelines(lines) ;
        >>> g = Graph(0); g.load("x.csv")
        >>> g
        * 1 1
        1 * 1
        1 1 *
        >>> import os; os.remove("x.csv")
        """
        if f.endswith(".csv"):
            loader = parse.load_csv
        elif f.endswith(".txt"):
            loader = parse.load_txt
        else:
            raise Exception("Unknown extention to find a loader")

        with open(f, "r") as f:
            loader(f, self.grow_with_edge)

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
        """
        Calcule la distance en précalculant la distance des sous-arbres,
        séléctionne les nœuds avec un sous-arbre ou à l'extrémité du graphe,
        et leur applique un parcours en largeur. Complexité maximale O(S*(S+A)),
        si le graphe est une forêt la complexité devient: O(S+A).
        """

        def printing(ms):
            if printer:
                print(ms, end="\x1b[1G")

        # Déctecte et calcule le diamètre pour les sous-arbre.
        printing(f"mark_tree")
        (whitelist, subtree, longest) = self.mark_tree()

        # Applique BFS sur chaque composante connexe.
        dist = [0] * self.n
        for n in range(self.n):
            if not whitelist[n] or dist[n] != 0:
                continue
            printing(f"first seen: {n}")
            for n, d in enumerate(self.bfs(n, whitelist)):
                if d != None:
                    dist[n] = d

        # Séléctionne les nœuds pouvant donner le diamètre.
        printing(f"selecting")
        origins = list(whitelist)
        for n in filter(lambda n: whitelist[n], range(self.n)):
            distN = dist[n]
            haveNotSubtree = subtree[n] == 0
            for c in self.children(n, whitelist):
                if distN < dist[c]:
                    if haveNotSubtree:
                        origins[n] = False
                elif subtree[c] == 0:
                    origins[c] = False

        # Récupère les nœuds séléctionnés et mesure le diamètre
        for origin, selected in enumerate(origins):
            if selected:
                printing(f"diameter: {n}")
                for n, d in enumerate(self.bfs(origin, whitelist)):
                    if d != None:
                        longest = max(longest, subtree[origin] + d + subtree[n])

        # Efface la ligne
        if printer:
            print("\x1b[K", end="")

        return longest

    def bfs(self, origin, whitelist):
        """Applique l'algorithme de parcours en largeur (*Breadth-first search*
        en anglais) sur le sommet `origin`. Complexite: O(A+S). La closure `f`
        prend le nœuds et sa distance minimal depuis l'origine. whitelist est
        un tableau permettant d'ignorées certains sommets."""
        dist = [None] * self.n
        dist[origin] = 0
        node_todo = [origin]

        while len(node_todo):
            parent = node_todo.pop(0)
            d = dist[parent]
            minimum = d + 1
            for child in self.children(parent, whitelist):
                if dist[child] == None:
                    dist[child] = minimum
                    node_todo.append(child)

        return dist

    def mark_tree(self):
        """
        Recherche tous les sous-arbres. Retourne un triplet:
        - Tableau des nœuds appartenant à des sous-arbres (plus pris en compte)
        - Tableau des poids des sous-arbres.
        - Distance maximal trouvée.
        >>> g = Graph(15)
        >>> for v in [(0, 1), (0, 2), (1, 2), (0, 3), (1, 4), (4, 5), (4, 6), (6, 7), (4, 8), (8, 10), (8, 9), (9, 11), (12, 13)]: g.add_edge(v)
        >>> (whitelist, weight, longest) = g.mark_tree()
        >>> whitelist
        [True, True, True, False, False, False, False, False, False, False, False, False, False, False, False]
        >>> weight[:3]
        [1, 4, 0]
        >>> longest
        5
        """
        whitelist = [True] * self.n  # Les nœuds appartenant à des sous-arbres
        weight = [0] * self.n  # Plus longue branche dans le sous-arbre.
        longest = 0  # La plus longue branche

        def nexter(it):
            "Retourne le prochain élément de l'itérateur ou None"
            try:
                return next(it)
            except StopIteration:
                return None

        for node in range(self.n):
            if not whitelist[node]:
                continue
            parent = node
            deep = 0
            while True:
                child = self.children(parent, whitelist)
                a = nexter(child)
                b = nexter(child)
                if a != None and b == None:
                    whitelist[parent] = False
                    parentDeep = weight[parent]
                    longest = max(longest, deep + parentDeep)
                    deep = 1 + max(deep, parentDeep)
                    parent = a
                elif a == None and b == None:
                    whitelist[parent] = False
                    longest = max(longest, deep)
                    break
                else:
                    parentDeep = weight[parent]
                    longest = max(longest, deep + parentDeep)
                    weight[parent] = max(deep, parentDeep)
                    break

        return (whitelist, weight, longest)

    def children(self, parent, whitelist):
        """
        Retourne un itérateur sur tous les voisins de parent qui sont à
        True sur la whiltelist.
        """
        return filter(lambda n: whitelist[n], self.adjacency_list[parent])


if __name__ == "__main__":

    def print_help():
        print("Usage de graph.py:")
        print()
        print("Charge ou genère le graphe avec:")
        print("    gg|gen_gilbert         size")
        print("    gb|gen_barabasi_albert size")
        print("    l|load                 file.csv|file.txt")
        print()
        print("Puis Indiquez un fichier si vous voulez exporter le graphe,")
        print("sinon ses statistiques seront affichées.")

    loader = sys.argv[1] if len(sys.argv) > 2 else ""
    if "-h" in sys.argv or "--help" in sys.argv or loader in ["help", ""]:
        print_help()
        quit()
    elif loader in ["gg", "gen_gilbert"]:
        g = Graph(int(sys.argv[2]))
        g.gen_gilbert()
    elif loader in ["gb", "gen_barabasi_albert"]:
        g = Graph(int(sys.argv[2]))
        g.gen_gilbert()
    elif loader == "load":
        g = Graph(0)
        g.load(sys.argv[2])
    else:
        print_help()
        print()
        sys.exit(f"Unknwon loader: '{loader}'")

    if len(sys.argv) > 3:
        g.save(sys.argv[3])
    else:
        g.stat()
