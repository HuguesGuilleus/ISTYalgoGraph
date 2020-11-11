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
                # calcule du degrer maximum et moyen
                self.calc_max_degree()
                self.calc_average_degree()

    def gen_barabasi_albert(self, m):
        n = self.n

        # initialisation de la clique
        self.add_edge([0, 1])
        self.add_edge([0, 2])
        self.add_edge([1, 2])

        self.nb_edges_tot = 3
        for i in range(3, n):
            cpt = 0
            j = 0
            while cpt < m and j < i:
                r = secrets.randbelow(self.nb_edges_tot)
                if r <= self.degree[j]:
                    self.add_edge([i, j])
                    cpt += 1
                j += 1
        #calcule du degrer maximum et moyen
        self.calc_max_degree()
        self.calc_average_degree()

    def calc_max_degree(self):
        self.max_degree = max(self.degree)

    def calc_average_degree(self):
        self.average_degree = sum(self.degree) / self.n

    #***************************************************
    #pas demander juste pour visuliser pour nous
    def curve_vertices_degree(self):
        x = []
        for i in range (0, self.n):
            x.append(i)
        y = self.degree

        plt.plot(x,y)
        plt.show()
    # ***************************************************

    def curve_distrib_degree(self):
        x = []
        y = []
        for i in range (0, self.max_degree+1):
            x.append(i)
            print(i)
            y.append(self.degree.count(i))
            print(self.degree.count(i))
            print()


        plt.bar(x,y)
        plt.show()