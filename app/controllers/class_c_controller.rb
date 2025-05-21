class ClassCController < ApplicationController
  def show
    @class_c = ClassC.find(params[:id])
    @problem = Problem.find(params[:p_id])
    @students = Student.where(classid: @class_c.id,  problem_sname: @problem.short_name)
  end
end
