require "test_helper"

class ProblemSetControllerTest < ActionDispatch::IntegrationTest
  test "should get show" do
    get problem_set_show_url
    assert_response :success
  end
end
