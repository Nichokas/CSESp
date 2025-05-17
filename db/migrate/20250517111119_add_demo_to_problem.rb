class AddDemoToProblem < ActiveRecord::Migration[8.0]
  def change
    add_column :problems, :demo, :string
  end
end
