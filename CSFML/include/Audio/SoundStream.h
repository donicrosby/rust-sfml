
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

#ifndef SFML_SOUNDSTREAM_H
#define SFML_SOUNDSTREAM_H

// Headers

#include "Audio/SoundStatus.h"
#include "Audio/Types.h"
#include "System/Vector3.h"
#include <cstdint>

typedef struct
{
    int16_t *samples;         ///< Pointer to the audio samples
    unsigned int sampleCount; ///< Number of samples pointed by Samples
} sfSoundStreamChunk;

typedef bool (*sfSoundStreamGetDataCallback)(sfSoundStreamChunk *, void *); ///< Type of the callback used to get a sound stream data
typedef void (*sfSoundStreamSeekCallback)(int64_t, void *);                 ///< Type of the callback used to seek in a sound stream

extern "C" sfSoundStream *sfSoundStream_create(sfSoundStreamGetDataCallback onGetData,
                                               sfSoundStreamSeekCallback onSeek,
                                               unsigned int channelCount,
                                               unsigned int sampleRate,
                                               void *userData);

extern "C" void sfSoundStream_destroy(sfSoundStream *soundStream);

extern "C" void sfSoundStream_play(sfSoundStream *soundStream);

extern "C" void sfSoundStream_pause(sfSoundStream *soundStream);

extern "C" void sfSoundStream_stop(sfSoundStream *soundStream);

extern "C" sfSoundStatus sfSoundStream_getStatus(const sfSoundStream *soundStream);

extern "C" unsigned int sfSoundStream_getChannelCount(const sfSoundStream *soundStream);

extern "C" unsigned int sfSoundStream_getSampleRate(const sfSoundStream *soundStream);

extern "C" void sfSoundStream_setPitch(sfSoundStream *soundStream, float pitch);

extern "C" void sfSoundStream_setVolume(sfSoundStream *soundStream, float volume);

extern "C" void sfSoundStream_setPosition(sfSoundStream *soundStream, sfVector3f position);

extern "C" void sfSoundStream_setRelativeToListener(sfSoundStream *soundStream, bool relative);

extern "C" void sfSoundStream_setMinDistance(sfSoundStream *soundStream, float distance);

extern "C" void sfSoundStream_setAttenuation(sfSoundStream *soundStream, float attenuation);

extern "C" void sfSoundStream_setPlayingOffset(sfSoundStream *soundStream, int64_t timeOffset);

extern "C" void sfSoundStream_setLoop(sfSoundStream *soundStream, bool loop);

extern "C" float sfSoundStream_getPitch(const sfSoundStream *soundStream);

extern "C" float sfSoundStream_getVolume(const sfSoundStream *soundStream);

extern "C" sfVector3f sfSoundStream_getPosition(const sfSoundStream *soundStream);

extern "C" bool sfSoundStream_isRelativeToListener(const sfSoundStream *soundStream);

extern "C" float sfSoundStream_getMinDistance(const sfSoundStream *soundStream);

extern "C" float sfSoundStream_getAttenuation(const sfSoundStream *soundStream);

extern "C" bool sfSoundStream_getLoop(const sfSoundStream *soundStream);

extern "C" int64_t sfSoundStream_getPlayingOffset(const sfSoundStream *soundStream);

#endif // SFML_SOUNDSTREAM_H
