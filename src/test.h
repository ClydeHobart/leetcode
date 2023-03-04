#pragma once

#include "assert.h"

namespace test
{
    typedef void (*Example)();

    struct ProblemData
    {
        const Example *examples;
        int problem;
    };

    void runExample(const ProblemData &problemData, int example);

    void runProblem(const ProblemData *problems, size_t size, int problem, int example);
}