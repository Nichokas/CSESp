class Problem < ApplicationRecord
  validates :name, presence: true
  validates :problem_description, presence: true
  validates :demo, presence: true
  validates :short_name, presence: true
end
