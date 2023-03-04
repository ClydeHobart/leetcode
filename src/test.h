#pragma once

#include <algorithm>
#include <iostream>

#include "assert.h"

namespace test
{
    typedef void (*Example)();

    struct ProblemData
    {
        const Example *examples;
        size_t size;
        int problem;
    };

    void runExample(const ProblemData &problemData, int example)
    {
        const auto printSuccess = [](int example)
        {
            std::cout << "\tExample " << (example + 1) << " completed successfully\n";
        };

        if (example > 0 && example <= problemData.size)
        {
            problemData.examples[example - 1]();
            printSuccess(example - 1);
        }
        else
        {
            for (size_t i = 0; i < problemData.size; ++i)
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
            std::cout << "Testing Problem " << problemData.problem << ":\n";
        };
        const auto problemData = std::find_if(
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