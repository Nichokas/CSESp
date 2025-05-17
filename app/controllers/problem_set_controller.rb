class ProblemSetController < ApplicationController
  def show
    @problem_set = ProblemSet.find(params[:id])
  end
end
