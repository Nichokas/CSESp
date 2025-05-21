class CreateStudents < ActiveRecord::Migration[8.0]
  def change
    create_table :students do |t|
      t.string :name
      t.string :problem_sname
      t.uuid :class_c_id
      t.timestamps
    end
  end
end
