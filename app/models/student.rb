class Student < ApplicationRecord
  validates :name , presence: true
  validates :problem_sname , presence: true
end
