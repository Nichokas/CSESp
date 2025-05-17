class CreateProblems < ActiveRecord::Migration[8.0]
  def change
    create_table :problems, if_not_exists: true do |t|
      t.string :name
      t.text :problem_description
      t.string :image
      t.string :demo
      t.string :files, array: true, default: []
      t.timestamps
    end
  end
end