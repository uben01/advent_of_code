import networkx as nx

edges = []
with open('../input.txt', 'r') as file:
    for line in file:
        edge = line.split('-')
        edges.append((edge[0], edge[1].strip()))

G = nx.Graph()
G.add_edges_from(edges)

longestClique = max(nx.find_cliques(G), key=len)
longestClique.sort()

print("Longest clique:", ','.join(longestClique))
