class UsersController < ApplicationController
  def new
    @user = User.new
  end

  def show 
    @user = User.find(params[:id])
  end

  def create 
    @user = User.new(user_params)

    @myparam = user_params[:account_id]

    @user.save 

    # Supposed to be "redirect_to user_url(@user)", but one got back /user.1 
    # instead of /users/1 which user_url should give the latter. Hopefully
    # this is just an error on my machine rather than some alien error. 
    
    @id =  User.find_by(account_id: user_params[:account_id]).id
    redirect_to "/users/#{@id}"
  end

  private 

    def user_params 
      params.require(:user).permit(:account_id, :public_key, :all_keys)
    end
end
