#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <math.h>

#define WINDOW_WIDTH 1366
#define WINDOW_HEIGHT 768

GC gc;

typedef struct
{
    int x;
    int y;
} Point2D;

typedef struct
{
    Point2D pos;
    u_int32_t color;
} Vertex2D;

// Structure to hold both array and its size
typedef struct
{
    float *values;
    int size;
} InterpResult;

typedef struct
{
    float alpha;
    float beta;
    // gamma = 1 - alpha - beta
} Barycentric;

typedef struct
{
    uint8_t alpha;
    uint8_t red;
    uint8_t green;
    uint8_t blue;
} color_ARGB;

// swaps two uint16_t values
void swap(uint16_t *val0, uint16_t *val1)
{
    uint16_t temp = *val0;
    *val0 = *val1;
    *val1 = temp;
}

// swaps two Point2D
void swapPoint2D(Point2D *P0, Point2D *P1)
{
    Point2D temp = *P0;
    *P0 = *P1;
    *P1 = temp;
}

// swaps two Vertex2D
void swapVertex2D(Vertex2D *P0, Vertex2D *P1)
{
    Vertex2D temp = *P0;
    *P0 = *P1;
    *P1 = temp;
}

// Red (255):   11111111  <- Shift left 16 bits
// Green (0):   00000000  <- Shift left 8 bits
// Blue (0):    00000000  <- No shift
// Result after shifts:
// 11111111 00000000 00000000  = 0xFF0000
uint32_t createRGB(u_int8_t red, u_int8_t green, u_int8_t blue)
{
    uint32_t color = 0; // 00000000 00000000 00000000  = 0x000000
    color = (red << 16) | (green << 8) | (blue);
    return color;
}
// Extract components
uint8_t getR(uint32_t color) { return (color >> 16) & 0xFF; }
uint8_t getG(uint32_t color) { return (color >> 8) & 0xFF; }
uint8_t getB(uint32_t color) { return color & 0xFF; }

// sets a pixel to a specific color in a given coordinate
bool setPixel(Display *_Display, Window _Window, Point2D P, uint32_t Color)
{
    XSetForeground(_Display, gc, Color);
    XDrawPoint(_Display, _Window, gc, P.x, P.y);
    return true;
}

InterpResult LinearInterpolation(uint16_t i0, uint16_t d0, uint16_t i1, uint16_t d1)
{
    // new empty result array {[], size}
    InterpResult result = {NULL, 0};

    // Special case: if points are the same, return array of size 1
    if (i0 == i1)
    {
        // if equal it means the line is straight
        result.values = (float *)malloc(sizeof(float));
        if (result.values == NULL)
        {
            return result; // Return with size 0 if malloc fails
        }
        // just use the same d for each i
        //(when horizontal use same height or vice versa)
        result.values[0] = d0;
        result.size = 1;
        return result;
    }

    // Calculate size needed (inclusive range)
    result.size = i1 - i0 + 1;

    // Allocate return array
    result.values = (float *)malloc(result.size * sizeof(float));
    if (result.values == NULL)
    {
        return result; // Return with size 0 if malloc fails
    }

    // calculate delta
    float a = (float)(d1 - d0) / (i1 - i0);

    // save starting point
    float d = d0;

    for (int i = 0; i < result.size; i++)
    {
        // save d
        result.values[i] = d;
        // with each iteration add another delta to d
        d = d + a;
    }

    return result;
}

InterpResult concatenateInterpResult(InterpResult first, InterpResult second, bool removeLast)
{

    InterpResult result = {NULL, first.size + second.size};

    if (removeLast)
    {
        result.size -= 1;
    }

    result.values = (float *)malloc(result.size * sizeof(float));
    if (result.values == NULL)
    {
        return result;
    }

    for (size_t i = 0; i <= first.size; i++)
    {
        result.values[i] = first.values[i];
    }

    // Copy second array
    size_t startIdx = 0;
    if (removeLast)
    {
        startIdx = first.size - 1;
    }
    else
    {
        startIdx = first.size;
    }

    for (size_t i = 0; i < second.size; i++)
    {
        result.values[first.size + i] = second.values[i];
    }

    return result;
}

// draws line between two points (x0,y0) and (x1,y1) with given color by using setPixel
bool drawLine(Display *_Display, Window _Window, Point2D P0, Point2D P1, uint32_t Color)
{
    if (abs(P1.x - P0.x) > abs(P1.y - P0.y))
    {
        // line is more horizontal then vertical
        // -> this must be true: x0 < x1

        if (P0.x > P1.x)
        {
            swapPoint2D(&P0, &P1);
        }

        // calculate the corrosponding y for each x
        InterpResult result = LinearInterpolation(P0.x, P0.y, P1.x, P1.y);

        if (result.values == NULL)
        {
            fprintf(stderr, "Memory allocation failed\n");
            return false;
        }

        // draw line by iterating through the results
        int i = 0;
        for (int x = P0.x; x <= P1.x; x++)
        {
            Point2D point = {x, result.values[i]};
            setPixel(_Display, _Window, point, Color);
            i++;
        }
    }
    else
    {

        // line is more vertical then horizontal
        // -> this must be true: y0 < y1

        if (P0.y > P1.y)
        {
            swapPoint2D(&P0, &P1);
        }

        // calculate the corrosponding x for each y
        InterpResult result = LinearInterpolation(P0.y, P0.x, P1.y, P1.x);

        if (result.values == NULL)
        {
            fprintf(stderr, "Memory allocation failed\n");
            return false;
        }

        // draw line by iterating through the results
        int i = 0;
        for (int y = P0.y; y <= P1.y; y++)
        {
            Point2D point = {result.values[i], y};
            setPixel(_Display, _Window, point, Color);
            i++;
        }
    }
    return true;
}

bool drawTriangle(Display *_Display, Window _Window, Point2D P0, Point2D P1, Point2D P2, uint32_t Color)
{

    // sort the y points such that y0 < y1 < y2
    if (P1.y < P0.y)
    {
        swapPoint2D(&P0, &P1);
    }
    if (P2.y < P0.y)
    {
        swapPoint2D(&P0, &P2);
    }
    if (P2.y < P1.y)
    {
        swapPoint2D(&P1, &P2);
    }

    // calculate boundaries of the triangle given by p0,p1,p2
    // we want the x values for each line between two points, thats why the independent value is y. y = i , x = d
    // naming: x01 -> x values between p0 and p1
    InterpResult x01 = LinearInterpolation(P0.y, P0.x, P1.y, P1.x);
    InterpResult x02 = LinearInterpolation(P0.y, P0.x, P2.y, P2.x);
    InterpResult x12 = LinearInterpolation(P1.y, P1.x, P2.y, P2.x);

    // if only one of those allocation fails, stop immediately
    if ((x01.values == NULL) || (x02.values == NULL) || (x12.values == NULL))
    {
        fprintf(stderr, "Memory allocation failed\n");
        return false;
    }

    // concatentate x01 and x12 so that we have two walls: x02 as the long wall and x012 as the bend other wall
    InterpResult x012 = {NULL, x01.size + x12.size};

    x012 = concatenateInterpResult(x01, x12, true);

    if (x012.values == NULL)
    {
        fprintf(stderr, "Memory allocation failed\n");
        return false;
    }

    // create left and right wall as x_left and x_right
    InterpResult x_left;
    InterpResult x_right;

    // check which wall is left and which is right
    // only check the middle since x012 is the wall bend
    size_t m = floor(x012.size / 2);
    if (x02.values[m] < x012.values[m])
    {
        x_left = x02;
        x_right = x012;
    }
    else
    {
        x_left = x012;
        x_right = x02;
    }

    // for every row from the left wall+1 to the right wall -1 set pixel to color
    for (int y = P0.y; y < P2.y; y++)
    {
        int row = y - P0.y;
        for (int x = x_left.values[row] + 1; x < x_right.values[row]; x++)
        {
            Point2D point = {x, y};
            setPixel(_Display, _Window, point, Color);
        }
    }

    // Free allocated memory
    free(x01.values);
    free(x02.values);
    free(x12.values);
    free(x012.values);
    return true;
}

float calcTriangleArea(Point2D P0, Point2D P1, Point2D P2)
{
    float signedArea = (P1.x - P0.x) * (P2.y - P0.y) - (P1.y - P0.y) * (P2.x - P0.x);
    return signedArea / 2.0f;
}

Barycentric calcBarycentricCoords(Point2D p, Point2D a, Point2D b, Point2D c)
{

    float abc_area = calcTriangleArea(a, b, c);
    float pbc_area = calcTriangleArea(p, b, c);
    float pca_area = calcTriangleArea(p, c, a);

    float alpha = pbc_area / abc_area;
    float beta = pca_area / abc_area;

    Barycentric result = {alpha, beta};
    return result;
}

bool drawGradientTriangle(Display *_Display, Window _Window, Vertex2D V0, Vertex2D V1, Vertex2D V2)
{

    // sort the y points such that y0 < y1 < y2
    if (V1.pos.y < V0.pos.y)
    {
        swapVertex2D(&V0, &V1);
    }
    if (V2.pos.y < V0.pos.y)
    {
        swapVertex2D(&V0, &V2);
    }
    if (V2.pos.y < V1.pos.y)
    {
        swapVertex2D(&V1, &V2);
    }

    // calculate boundaries of the triangle given by p0,p1,p2
    // we want the x values for each line between two points, thats why the independent value is y. y = i , x = d
    // naming: x01 -> x values between p0 and p1
    InterpResult x01 = LinearInterpolation(V0.pos.y, V0.pos.x, V1.pos.y, V1.pos.x);
    InterpResult x02 = LinearInterpolation(V0.pos.y, V0.pos.x, V2.pos.y, V2.pos.x);
    InterpResult x12 = LinearInterpolation(V1.pos.y, V1.pos.x, V2.pos.y, V2.pos.x);

    // if only one of those allocation fails, stop immediately
    if ((x01.values == NULL) || (x02.values == NULL) || (x12.values == NULL))
    {
        fprintf(stderr, "Memory allocation failed\n");
        return false;
    }

    // concatentate x01 and x12 so that we have two walls: x02 as the long wall and x012 as the bend other wall
    InterpResult x012 = {NULL, x01.size + x12.size};

    x012 = concatenateInterpResult(x01, x12, true);

    if (x012.values == NULL)
    {
        fprintf(stderr, "Memory allocation failed\n");
        return false;
    }

    // create left and right wall as x_left and x_right
    InterpResult x_left;
    InterpResult x_right;

    // check which wall is left and which is right
    // only check the middle since x012 is the wall bend
    size_t m = floor(x012.size / 2);
    if (x02.values[m] < x012.values[m])
    {
        x_left = x02;
        x_right = x012;
    }
    else
    {
        x_left = x012;
        x_right = x02;
    }

    // Calculate area once per triangle
    float abc_area = calcTriangleArea(V0.pos, V1.pos, V2.pos);

    // Precompute some constant terms used in barycentric calculation
    float x0 = V0.pos.x;
    float y0 = V0.pos.y;
    float x1 = V1.pos.x;
    float y1 = V1.pos.y;
    float x2 = V2.pos.x;
    float y2 = V2.pos.y;

        
    // These terms stay constant for the triangle
    // in naive it this will be called very often:
    // float signedArea = (P1.x - P0.x) * (P2.y - P0.y) - (P1.y - P0.y) * (P2.x - P0.x);
    // we can precalculate these steps:
    float v0x = x1 - x0; // (P1.x - P0.x)
    float v0y = y1 - y0; // (P1.y - P0.y)
    float v1x = x2 - x0; // (P2.x - P0.x) 
    float v1y = y2 - y0; // (P2.y - P0.y)
    
    // and this float signedArea = (P1.x - P0.x) * (P2.y - P0.y) - (P1.y - P0.y) * (P2.x - P0.x);
    // with new terms but as 1/signedArea 
    // so we only need to do the division once
    float denom = 1.0f / (v0x * v1y - v1x * v0y); 

    // for every row from the left wall+1 to the right wall -1 set pixel to color
    for (int y = V0.pos.y; y < V2.pos.y; y++)
    {
        int row = y - V0.pos.y;
        for (int x = x_left.values[row] + 1; x < x_right.values[row]; x++)
        {
            Point2D point = {x, y};

            // Calculate barycentric coordinates more efficiently
            float px = x - x0; // x distance from vertex 0 to current pixel
            float py = y - y0; // y distance from vertex 0 to current pixel
            
            float alpha = (px * v1y - py * v1x) * denom; // Area(pbc) * (1/Area(abc))
            float beta = (py * v0x - px * v0y) * denom;  // Area(pca) * (1/Area(abc))

            uint8_t r = alpha * getR(V0.color) + beta * getR(V1.color) + (1 - alpha - beta) * getR(V2.color);
            uint8_t g = alpha * getG(V0.color) + beta * getG(V1.color) + (1 - alpha - beta) * getG(V2.color);
            uint8_t b = alpha * getB(V0.color) + beta * getB(V1.color) + (1 - alpha - beta) * getB(V2.color);

            setPixel(_Display, _Window, point, createRGB(r, g, b));
        }
    }
    return true;
}

int main()
{
    Display *display;
    Window window;
    XEvent event;
    int screen;

    // Open connection to X server
    display = XOpenDisplay(NULL);
    if (display == NULL)
    {
        fprintf(stderr, "Cannot open display\n");
        exit(1);
    }

    // Get default screen
    screen = DefaultScreen(display);

    // Create window
    window = XCreateSimpleWindow(display, RootWindow(display, screen),
                                 100, 100, WINDOW_WIDTH, WINDOW_HEIGHT, 1,
                                 BlackPixel(display, screen),
                                 BlackPixel(display, screen));

    gc = XCreateGC(display, window, 0, NULL);

    // Select kinds of events we are interested in
    XSelectInput(display, window, ExposureMask | KeyPressMask);

    XSetForeground(display, gc, BlackPixel(display, screen));

    // Show the window
    XMapWindow(display, window);

    // Event loop
    while (1)
    {
        XNextEvent(display, &event);

        // Handle events
        switch (event.type)
        {
        case Expose:
            // Window needs to be redrawn

            Vertex2D V0 = {{200, WINDOW_HEIGHT - 100}, createRGB(255, 0, 0)};
            Vertex2D V1 = {{WINDOW_WIDTH/2, 100}, createRGB(0, 255, 0)};
            Vertex2D V2 = {{WINDOW_WIDTH - 200, WINDOW_HEIGHT - 100}, createRGB(0, 0, 255)};

            drawLine(display, window, V0.pos, V1.pos, 0);
            drawLine(display, window, V1.pos, V2.pos, 0);
            drawLine(display, window, V2.pos, V0.pos, 0);

            // drawTriangle(display, window, V0.pos, V1.pos, V2.pos, createRGB(0, 0, 255));

            drawGradientTriangle(display, window, V0, V1, V2);

            break;
        case KeyPress:
            // Exit on any key press
            // goto cleanup;
            break;
        }
    }

cleanup:
    // Close connection to X server
    XCloseDisplay(display);

    return 0;
}