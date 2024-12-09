def bresenham(a_x, a_y, b_x, b_y):
    swapped = False
    # Berechne den Anstieg m
    m = (b_y - a_y) / (b_x - a_x)
    print(f"m = {m}")


    # Überprüfe, ob m außerhalb von -1 und 1 liegt, wenn doch...
    if not -1 <= m <= 1:
        # dann vertausche x- und y-Koordinaten
        a_x, a_y = a_y, a_x
        b_x, b_y = b_y, b_x

        print("m zu steil, swappe x und y...")
        swapped = True

        # Berechne m neu
        m = (b_y - a_y) / (b_x - a_x)
        print(f"neues m = {m}")

    # Berechne den Wert von t
    t = a_y - (m * a_x)

    # Gebe t aus
    print(f"t = {t} = {a_y} - ({m} * {a_x})")

    # Runde m * x + t und gib es aus
    print(f"{round(m * a_x + t)} = round({m} * {a_x} + {t})")

    # Berechne y für jedes x zwischen a_x und b_x
    for x in range(a_x, b_x + 1):

        pixel_x = x
        pixel_y = round(m * x + t)

        if swapped: # Wenn vorher getauscht wurde, dann tausche x und y zurück
            pixel_x, pixel_y = pixel_y, pixel_x
            
        setpixel(pixel_x, pixel_y)

def setpixel(x,y):
    print(f"({x}, {y})")


# Testen der Funktion mit den gegebenen Werten
a_x = -1
a_y = 11
b_x = 2
b_y = 21
bresenham(a_x, a_y, b_x, b_y)
#bresenham(a_y, a_x, b_y, b_x)
