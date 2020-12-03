from python.graph import Graph

# ******************************
# **     fichier de test      **
# ******************************

g = Graph(2500)
g.gen_barabasi_albert(3)
g.stat()
