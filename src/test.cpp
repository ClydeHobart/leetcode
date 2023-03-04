#include "test.h"

#include <algorithm>
#include <iostream>

using namespace std;

namespace test
{
    void runExample(const ProblemData &problemData, int example)
    {
        const auto printSuccess = [](int example)
        {
            cout << "\tExample " << (example + 1) << " completed successfully\n";
        };

        size_t size = 0;

        if (problemData.examples)
        {
            while (problemData.examples[size])
            {
                ++size;
            };
        }

        if (example > 0 && example <= size)
        {
            problemData.examples[example - 1]();
            printSuccess(example - 1);
        }
        else
        {
            for (size_t i = 0; i < size; ++i)
            {
                problemData.examples[i]();
                printSuccess(i);
            }
        }
    }

    void runProblem(const ProblemData *problems, size_t size, int problem, int example)
    {
        const auto printProblemHeader = [](const ProblemData &problemData)
        {
            cout << "Testing Problem " << problemData.problem << ":\n";
        };
        const auto problemData = find_if(
            problems,
            problems + size,
            [=](const ProblemData &problemData) -> bool
            { return problemData.problem == problem; });

        if (problemData != problems + size)
        {
            printProblemHeader(*problemData);
            runExample(*problemData, example);
        }
        else
        {
            for (size_t i = 0; i < size; ++i)
            {
                const ProblemData &problemData = problems[i];
                printProblemHeader(problemData);
                runExample(problemData, example);
            }
        }
    }
}