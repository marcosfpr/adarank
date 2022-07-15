#include <cstdlib>
#include <iostream>

#include <ltr.hpp>

#include <gtest/gtest.h>

using namespace ltr;

TEST(test_dataset, parsing) {
    char* env_dir = std::getenv("OHSUMED");

    ASSERT_TRUE(env_dir != nullptr);

    std::string base_path(env_dir);

    DataSet training_samples = load_svmlight(base_path + "/Data/All/OHSUMED.txt");

    ASSERT_EQ(training_samples.size(), 106); // there are 106 queries on OHSUMED dataset
}