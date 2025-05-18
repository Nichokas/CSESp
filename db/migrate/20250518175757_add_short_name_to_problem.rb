class AddShortNameToProblem < ActiveRecord::Migration[8.0]
  def change
    add_column :problems, :short_name, :string
  end
end
