Rails.application.routes.draw do
  get "tutorial_autocorreccion", to: "tutorial_autocorreccion#index"
  get "problem/:id", to: "problem#show"
  get "problem_set/:id", to: "problem_set#show"
  get "problem/:p_id/:id", to: "class_c#show"
  root "homepage#index"

  # Reveal health status on /up that returns 200 if the app boots with no exceptions, otherwise 500.
  # Can be used by load balancers and uptime monitors to verify that the app is live.
  get "up" => "rails/health#show", as: :rails_health_check

  mount ActionCable.server => "/submit_cli"

end
