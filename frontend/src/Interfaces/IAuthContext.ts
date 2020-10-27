export interface IAuthContext {
    authed: boolean,
    userId: string,
    token: string,
    loading: boolean
}

export type TSetAuthContext = (state: IAuthContext) => void;