require_dependency "conversation_problem_helper"
require_dependency "problem_solver"

class SubmitCliChannel < ApplicationCable::Channel
  def subscribed
    stream_from "submit_cli_#{connection.connection_identifier || SecureRandom.uuid}"

    @c_state = :initial
    @problem_sname = nil
    @assumption_storage = nil
    @challenge_storage = nil
    transmit({ message: "*/**/Connected to server/**/*" })
  end

  def unsubscribed
    @stored_message = nil
    @assumption_storage = nil
  end

  def receive(data)
    message_text = data["message"]

    case @c_state
    when :initial
      # check if the problem actually exists
      if ConversationProblemHelper.find(message_text)
        @problem_sname = message_text
        transmit({ message: "*/**/Problem selected/**/*" })

        # first try
        @challenge_storage=ConversationProblemHelper.g_test(@problem_sname)
        @assumption_storage=ProblemSolver.public_send(@problem_sname.to_sym, @challenge_storage)
        @c_state = :waiting_1st_challenge
        transmit({ message: "#{@challenge_storage}" })
      else
        transmit({ message: "*/**/Problem not found/**/*" })
        @c_state = :initial # prevent program to continue
      end
    when :waiting_1st_challenge

      if message_text == @assumption_storage
        transmit({ message: "*/**/Correct/**/*" })

        # if correct, go with the second challenge
        @challenge_storage=ConversationProblemHelper.g_test(@problem_sname)
        @assumption_storage=ProblemSolver.public_send(@problem_sname.to_sym, @challenge_storage)
        transmit({ message: "#{@challenge_storage}" })
        @c_state = :waiting_2nd_challenge
      else
        transmit({ message: "*/**/Solution Err/**/*" })
      end
    when :waiting_2nd_challenge
      if message_text == @assumption_storage
        transmit({ message: "*/**/Correct/**/*" })
        transmit({ message: "*/**/Solution Validated Succesfully/**/*" })
      else
        transmit({ message: "*/**/Solution Err/**/*" })
        @c_state = :initial # prevent program to continue
      end
      end
    end
  end
