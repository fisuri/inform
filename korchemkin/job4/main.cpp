#include <iostream>
#include <cstdlib>
using namespace std;

struct student
{
    string name;
    int assessements[3][3];
    int zachet;
    double usp1[3];
};

int randint(int min, int max)
{
    return rand() % (max - min + 1) + min;
}

int main()
{
    srand(time(NULL));

    student g;
    cout << "Ima studenta = ";
    cin >> g.name;

    for (int i = 0; i < 3; i++)
    {
        for (int j = 0; j < 3; j++)
        {
            g.assessements[i][j] = randint(1, 12);
        }
    }

    for (int i = 0; i < 3; i++)
    {
        g.usp1[i] = (g.assessements[i][0] + g.assessements[i][1] + g.assessements[i][2]) / 3;
    }

    g.zachet = randint(10000, 99999);

    for (int i = 0; i < 3; i++)
    {
        for (int j = 0; j < 2; j++)
        {
            if (g.usp1[j] > g.usp1[j + 1])
            {
                int repka = g.usp1[j];
                g.usp1[j] = g.usp1[j + 1];
                g.usp1[j + 1] = repka;
            }
        }
    }

    cout << "Uspevaemost studente = " << g.name << endl;

    cout << "Zachet = " << g.zachet << endl;

    for (int i = 0; i < 3; i++)
    {
        for (int j = 0; j < 3; j++)
        {
            cout << g.assessements[i][j] << " ";
        }
    }

    for (int i = 0; i < 1; i++)
    {
        cout << endl
             << "ABS assessement = " << g.usp1[0] << " " << g.usp1[1] << " " << g.usp1[2] << endl;
    }

    return 0;
}