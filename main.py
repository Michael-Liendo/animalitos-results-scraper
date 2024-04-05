""" import pandas as pd
import matplotlib.pyplot as plt

# Read the CSV
datos_loteria = pd.read_csv("results.csv")

# Convert the "date" column to date type (assuming "date" instead of "fecha")
datos_loteria["date"] = pd.to_datetime(datos_loteria["date"])

# Group data by hour and animal, get frequency
animales_por_hora = datos_loteria.groupby(["hour", "animal"]).size().unstack()

# Option 1: Plot each animal's frequency as a separate bar
animales_por_hora.plot(kind="bar", stacked=False)  # Change stacked to False

# Option 2: Plot total frequency per hour (sum each row)
animales_por_hora.sum(axis=1).plot(kind="bar")

# Customize the plot (labels, title, etc.)
plt.xlabel("Hora")
plt.ylabel("Frecuencia")
plt.title("Frecuencia de animales por hora en la lotería")
plt.show()
"""

import pandas as pd

# Leer el archivo CSV
datos = pd.read_csv("results.csv")

# Contar las apariciones de cada elemento
conteo = datos["animal"].value_counts()

# Calcular la frecuencia relativa
frecuencia_relativa = conteo / conteo.sum()

# Multiplicar por 100 para obtener el porcentaje
probabilidad = frecuencia_relativa * 100

# Mostrar la probabilidad de cada elemento
for elemento, probabilidad in zip(conteo.index, probabilidad):
    print(f"Elemento {elemento}: {probabilidad:.2f}%")