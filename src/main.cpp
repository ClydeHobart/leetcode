#include <iostream>

#include "p34.h"
#include "p420.h"
#include "p1276.h"

using namespace std;
using namespace test;

#define ARRAY_SIZE(x) sizeof(x) / sizeof(*x)
#define LIST_PROBLEMS(f) \
    f(34)                \
        f(420)           \
            f(1276)

#define DEFINE_PROBLEM_DATA(problem) ProblemData{p##problem::examples, problem},

static const ProblemData problems[] = {
    LIST_PROBLEMS(DEFINE_PROBLEM_DATA)};

bool isAllDigits(const char *src)
{
    for (size_t i = 0; src[i]; ++i)
    {
        const char c = src[i];

        if (c < '0' || c > '9')
        {
            return false;
        }
    }

    return true;
}

int main(int argc, char *argv[])
{
    static const char usage[] = "Usage: leetcode [<problem> [<example>]]\n";
    int problem = -1;
    int example = -1;

    if (argc > 3)
    {
        cerr << "Too many arguments.\n\n"
             << usage;

        return 1;
    }
    else if (argc > 1)
    {
        const char *problemArg = argv[1];

        if (!isAllDigits(problemArg))
        {
            cerr << "Invalid <problem> argument. It should be a decimal number.\n\n"
                 << usage;

            return 2;
        }

        problem = atoi(problemArg);

        if (argc > 2)
        {
            const char *exampleArg = argv[2];

            if (!isAllDigits(exampleArg))
            {
                cerr << "Invalid <example> argument. It should be a decimal number.\n\n"
                     << usage;

                return 3;
            }

            example = atoi(exampleArg);
        }
    }

    runProblem(problems, ARRAY_SIZE(problems), problem, example);

    return 0;
}