export const idlFactory = ({ IDL }) => {
  const Result = IDL.Record({ 'ok' : IDL.Bool, 'payload' : IDL.Text });
  const GameState = IDL.Variant({
    'New' : IDL.Null,
    'Answered' : IDL.Null,
    'Asked' : IDL.Null,
  });
  const Game = IDL.Record({
    'id' : IDL.Nat32,
    'found' : IDL.Vec(IDL.Bool),
    'answers' : IDL.Vec(IDL.Text),
    'score' : IDL.Nat32,
    'keywords' : IDL.Vec(IDL.Text),
    'state' : GameState,
    'questions' : IDL.Vec(IDL.Text),
    'principal_str' : IDL.Text,
    'points' : IDL.Vec(IDL.Nat8),
  });
  const Profile = IDL.Record({
    'name' : IDL.Text,
    'max_score' : IDL.Nat32,
    'games' : IDL.Vec(IDL.Nat32),
  });
  return IDL.Service({
    'addAnswers' : IDL.Func([IDL.Nat32, IDL.Vec(IDL.Text)], [Result], []),
    'addQuestions' : IDL.Func([IDL.Nat32, IDL.Vec(IDL.Text)], [Result], []),
    'clearAll' : IDL.Func([], [], []),
    'getAsked' : IDL.Func([], [IDL.Vec(IDL.Nat32)], ['query']),
    'getGame' : IDL.Func([IDL.Nat32], [Game], ['query']),
    'getSelf' : IDL.Func([], [Profile], ['query']),
    'listAllGames' : IDL.Func([], [IDL.Vec(Game)], ['query']),
    'startGame' : IDL.Func([], [Result], []),
    'update' : IDL.Func([IDL.Text], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
