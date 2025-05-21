class AddClassToStudents < ActiveRecord::Migration[8.0]
  def change
    add_column :students , :classid, :uuid
  end
end
