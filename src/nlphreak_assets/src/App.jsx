import React, { useState, useEffect } from 'react';

import Footer from './components/Footer';
import Header from './components/Header';
import MainBody from './components/MainBody';

import { nlphreak } from "../../declarations/nlphreak";

import { createActor, canisterId } from "../../declarations/nlphreak"
import { AuthClient } from "@dfinity/auth-client"
import Username from './components/Username';
import Game from './components/Game';

import useLocalStorage from './hooks/useLocalStorage'

function App() {

    const [actor, setActor] = useState(nlphreak);
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const [checkLoginStatus, setCheckLoginStatus] = useState(false);

    useEffect(async () => {
        let authClient = await AuthClient.create();
        const aaa = await authClient?.isAuthenticated();
        setIsLoggedIn(aaa);

        //console.log(aaa);

        //console.log(authClient.getIdentity());

        const identity = authClient.getIdentity();

        setClientIdentity(identity.getPrincipal().toString());

        const zzz = createActor(canisterId, { agentOptions: { identity: identity } });
        //console.log(zzz)
        setActor(zzz);
    }, [checkLoginStatus])

    const [clientIdentity, setClientIdentity] = useState("")

    const [startedGame, setStartedGame] = useLocalStorage(0)

    return (
        <div>
            <h6>Step1:</h6>
            <div>{isLoggedIn ? <>Logged in as {clientIdentity}</> : <>
                <button onClick={
                    async () => {
                        //const iiUrl = "http://localhost:8000/?canisterId=rkp4c-7iaaa-aaaaa-aaaca-cai";
                        //const iiUrl = "http://localhost:8000/?canisterId=rwlgt-iiaaa-aaaaa-aaaaa-cai";
                        const iiUrl = "https://identity.ic0.app/#authorize";

                        let authClient = await AuthClient.create();
                        let zz = await authClient.login({
                            identityProvider: iiUrl,
                            maxTimeToLive: BigInt(9_000_000_000_000_000_000),
                            onSuccess: () => {
                                setCheckLoginStatus(true);
                            },
                        });



                    }}>Login</button>
            </>}</div>

            <h6>Step2:</h6>
            <Username actor={actor}></Username>

            <h6>Step3:</h6>
            <button onClick={async () => {
                const x = await actor.startGame();
                console.log(x)
                setStartedGame(parseInt(x.payload))
            }}>StartNewGame</button>

            {startedGame > 1 ? <>Started Game {startedGame}

                <Game actor={actor} id={startedGame}></Game>

            </> : <></>}
        </div>
    )
}

export default App
