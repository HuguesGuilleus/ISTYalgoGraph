class Graph:
    """Un graphe, il contient une liste où chaque sommet a ses enfants."""

    def __init__(self, size):
        self.adjacency_list = []
        for i in range(size or 0):
            self.adjacency_list.append([])

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

    def len(self):
        """Retourne le nombre de sommet du graphe. Complexité O(1)"""
        return len(self.adjacency_list)
