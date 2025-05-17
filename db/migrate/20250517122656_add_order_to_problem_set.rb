class AddOrderToProblemSet < ActiveRecord::Migration[8.0]
  def change
    add_column :problem_sets, :order, :integer
  end
end
