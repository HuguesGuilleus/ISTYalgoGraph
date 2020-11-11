import secrets


class Graph:
    def __init__(self, size):
        self.n = size
        self.edges = []
        self.nb_edges_tot = 0
        self.degree = []
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
        n = self.n

        # initialisation de la clique
        self.degree[0] = 1
        self.degree[0] = 1
        self.degree[0] = 1
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
