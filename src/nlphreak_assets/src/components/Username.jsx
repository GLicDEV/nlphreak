import React, { useState, useEffect } from 'react';

const Username = (props) => {

    const [selfResponse, setSelfResponse] = useState("")
    const [chosenUserName, setChosenUserName] = useState("")


    useEffect(async () => {
        const self = await props.actor.getSelf();
        console.log(self)
        setSelfResponse(self)
    }, [props, chosenUserName])

    const updateUserName = async (event) => {
        event.preventDefault();

        console.log(chosenUserName)

        const x = await props.actor.update(chosenUserName);
        console.log(x)

    }

    return (
        <div>
            {selfResponse.name === "" ? <> Please choose a username
                <form onSubmit={updateUserName}>
                    <input name="userName" onChange={(e) => setChosenUserName(e.target.value)}></input>
                    <button type="submit">Submit</button>
                </form>
            </> : <> Your username is {selfResponse.name} </>}
        </div>
    )
}

export default Username
