class AddHintsToProblems < ActiveRecord::Migration[8.0]
  def change
    add_column :problems, :hints, :string, array: true, default: []
  end
end
