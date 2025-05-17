class HomepageController < ApplicationController
  def index
    @problem_sets = ProblemSet.all
  end
end
