require "test_helper"

class ProblemSetTest < ActiveSupport::TestCase
  test "can store an array of exercises" do
    ps = ProblemSet.create!(name: "Set 1", description: "Desc", exercises: ["ex1", "ex2"])
    assert_equal ["ex1", "ex2"], ps.exercises
  end
end