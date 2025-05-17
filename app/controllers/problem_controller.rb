class ProblemController < ApplicationController
  def show
    @problem = Problem.find(params[:id])
  end
end
