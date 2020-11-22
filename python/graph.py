import secrets
import matplotlib.pyplot as plt


class Graph:
    def __init__(self, size):
        self.n = size
        self.edges = []
        self.nb_edges_tot = 0
        self.degree = []
        self.max_degree = 0
        self.average_degree = 0
        # initialisation des degrees des sommets à 0
        for i in range(0, size):
            self.degree.append(0)

    def __str__(self):
        s = ""
        for i in range(0, len(self.edges)):
            s = s + "," + str(self.edges[i])
        return s

    def add_edge(self, vertices):
        # ajouter les sommets au tableau des aretes
        self.degree[vertices[0]] += 1
        self.degree[vertices[1]] += 1
        self.nb_edges_tot += 1
        self.edges.append(vertices)

    def gen_gilbert(self):
        n = self.n
        for i in range(0, n):
            for j in range(i + 1, n):
                r = secrets.randbits(1)
                if r:
                    self.add_edge([i, j])

    def gen_barabasi_albert(self, m):
        # initialisation de la clique
        self.add_edge([0, 1])
        self.add_edge([0, 2])
        self.add_edge([1, 2])
        self.nb_edges_tot = 3

        for i in range(3, self.n):
            cpt = 0
            j = 0
            while cpt < m and j < i:
                r = secrets.randbelow(self.nb_edges_tot * 2)
                if r <= self.degree[j]:
                    self.add_edge([i, j])
                    cpt += 1
                j += 1

    def calc_max_degree(self):
        self.max_degree = max(self.degree)

    def calc_average_degree(self):
        self.average_degree = sum(self.degree) / self.n

    def curve_distrib_degree(self):
        x = []
        y = [0] * (self.max_degree + 1)
        for i in range(0, self.max_degree + 1):
            x.append(i)
        for i in range(0, self.n):
            y[self.degree[i]] += 1
        plt.bar(x, y)
        plt.show()

    def stat(self):
        print("1) le nombre de sommets est : " + str(self.n))
        print("2) le nombre d'arêtes est : " + str(self.nb_edges_tot))
        self.calc_max_degree()
        print("3) le degrée maximal est : " + str(self.max_degree))
        self.calc_average_degree()
        print("4) le degrée moyen est : " + str(self.average_degree))
        print("6) le diamètre du graph est : " + "???")
        print("5) la courbe de distribution des degrées s'ouvre dans une fenêtre")
        self.curve_distrib_degree()
