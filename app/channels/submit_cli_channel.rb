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
        @challenge_storage = ConversationProblemHelper.g_test(@problem_sname)
        @assumption_storage = ProblemSolver.public_send(@problem_sname.to_sym, @challenge_storage)
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
        @challenge_storage = ConversationProblemHelper.g_test(@problem_sname)
        @assumption_storage = ProblemSolver.public_send(@problem_sname.to_sym, @challenge_storage)
        transmit({ message: "#{@challenge_storage}" })
        @c_state = :waiting_2nd_challenge
      else
        transmit({ message: "*/**/Solution Err/**/*" })
      end
    when :waiting_2nd_challenge
      if message_text == @assumption_storage
        transmit({ message: "*/**/Correct/**/*" })
        transmit({ message: "*/**/Solution Validated Successfully/**/*" })
        @c_state = :register
      else
        transmit({ message: "*/**/Solution Err/**/*" })
        @c_state = :initial # prevent program to continue
      end
    when :register
      things = message_text.split(",")
      student = Student.new(
        name: things[0].strip,
        problem_sname: things[2].strip,
        classid: things[1].strip,
        )

      if ClassC.find_by_id(student.classid).present?
        if student.valid?
          if student.save
            transmit({ message: "*/**/Guardado correctamente/**/*" })
            @c_state = :initial
          else
            transmit({ message: "*/**/Error: #{student.errors.full_messages.join(', ')}/**/*" })
            @c_state = :initial
          end
        else
          transmit({ message: "*/**/Datos de estudiante/clase no validos/**/*" })
        end
      else
        transmit({ message: "*/**/Datos de estudiante/clase no validos/**/*" })
      end
    end
  end
end
