class CreateProblems < ActiveRecord::Migration[8.0]
  def change
    create_table :problems do |t|
      t.string :name
      t.text :steps
      t.string :files, array: true, default: []

      t.timestamps
    end
  end
end
