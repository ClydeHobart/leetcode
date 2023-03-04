#pragma once

#include <cstdio>
#include <cstdlib>

#define assert(...) ((__VA_ARGS__) || (fprintf(stderr, "Expression \"" #__VA_ARGS__ "\" on line %d of file %s was false. Aborting.\n", __LINE__, __FILE__), abort(), false))