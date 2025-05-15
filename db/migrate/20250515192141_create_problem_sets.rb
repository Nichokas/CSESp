class CreateProblemSets < ActiveRecord::Migration[8.0]
  def change
    create_table :problem_sets do |t|
      t.string :name
      t.text :description
      t.string :exercises, array: true, default: []

      t.timestamps
    end
  end
end
