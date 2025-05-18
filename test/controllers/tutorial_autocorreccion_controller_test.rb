require "test_helper"

class TutorialAutocorreccionControllerTest < ActionDispatch::IntegrationTest
  test "should get index" do
    get tutorial_autocorreccion_index_url
    assert_response :success
  end
end
