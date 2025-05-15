class ProblemSet < ApplicationRecord
  validates :name, presence: true
  validates :description, presence: true
  validates :exercises, presence: true
end
